//! Primite Dragon Nether Berzelius
use super::prelude::*;

pub const ID: u32 = 26462013;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.listed_series={SET_PRIMITE}
    listed_series: &[SET_PRIMITE],
    // s.material_setcode={SET_PRIMITE}
    material_setcode: &[SET_PRIMITE],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: 1 "Primite" monster + 1+ Normal Monsters
    // Fusion.AddProcMixRep(c,true,true,aux.FilterBoolFunctionEx(Card.IsType,TYPE_NORMAL),1,99,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_PRIMITE))
    fusion::add_proc_mix_rep(
        d,
        c,
        true,
        true,
        fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| card::is_type_ctx(d, x, TYPE_NORMAL, Some(fc), st, Some(tp)))),
        1,
        99,
        vec![fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| card::is_set_card_ctx(d, x, &[SET_PRIMITE as u16], Some(fc), st, Some(tp))))],
    );
    // Gains 1000 ATK for each Normal Monster used for its Fusion Summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e1, EFFECT_MATERIAL_CHECK);
    // e1:SetValue(function(e,c) c:UpdateAttack(...) end)
    effect::set_value_fn(d, e1, effect_card_value(atkval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Negate the activated effects of monsters your opponent controls whose Level/Rank/Link Rating is less than or equal to the number of Normal Monsters in your field and GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_CHAIN_SOLVING)
    effect::set_code(d, e2, EVENT_CHAIN_SOLVING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.discon)
    effect::set_condition(d, e2, Some(condition(discon)));
    // e2:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) Duel.Hint(HINT_CARD,0,id) Duel.NegateEffect(ev) end)
    effect::set_operation(d, e2, Some(operation(negop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon 1 Normal Monster from your Deck in Defense Position
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e3:SetCondition(function(e) return e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD) end)
    effect::set_condition(d, e3, Some(condition(cond3)));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.discon(e,tp,eg,ep,ev,re,r,rp)
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // if not re:IsMonsterEffect() or e:GetHandler():IsStatus(STATUS_BATTLE_DESTROYED) then return false end
    if !effect::is_monster_effect(d, a.re.nil()) {
        return false;
    }
    let h = effect::get_handler(d, a.e).nil();
    if card::is_status(d, h, STATUS_BATTLE_DESTROYED) {
        return false;
    }
    // local normal_ct=Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsType,TYPE_NORMAL),tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil)
    let f = aux::faceup_filter(|d, x| card::is_type(d, x, TYPE_NORMAL));
    let normal_ct = duel::get_matching_group_count(d, Some(&f), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX) as i64;
    // if normal_ct==0 then return false end
    if normal_ct == 0 {
        return false;
    }
    // local trig_ctrl,trig_loc,trig_lv,trig_rk=Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_CONTROLER,CHAININFO_TRIGGERING_LOCATION,CHAININFO_TRIGGERING_LEVEL,CHAININFO_TRIGGERING_RANK)
    let info = duel::get_chain_info(
        d,
        a.ev as u8,
        &[CHAININFO_TRIGGERING_CONTROLER, CHAININFO_TRIGGERING_LOCATION, CHAININFO_TRIGGERING_LEVEL, CHAININFO_TRIGGERING_RANK],
    );
    let (trig_ctrl, trig_loc, trig_lv, trig_rk) = (info[0].int(), info[1].int(), info[2].int(), info[3].int());
    // if not (trig_ctrl==1-tp and trig_loc==LOCATION_MZONE) then return false end
    if !(trig_ctrl == (1 - a.tp) as i64 && trig_loc == LOCATION_MZONE as i64) {
        return false;
    }
    // local trig_lk=re:GetHandler():GetLink()
    let rh = effect::get_handler(d, a.re.nil()).nil();
    let trig_lk = card::get_link(d, rh) as i64;
    // if trig_lv>0 and trig_lv<=normal_ct then return true end
    if trig_lv > 0 && trig_lv <= normal_ct {
        return true;
    }
    // if trig_rk>0 and trig_rk<=normal_ct then return true end
    if trig_rk > 0 && trig_rk <= normal_ct {
        return true;
    }
    // if trig_lk>0 and trig_lk<=normal_ct then return true end
    if trig_lk > 0 && trig_lk <= normal_ct {
        return true;
    }
    // return false
    false
}
// function(e,tp,eg,ep,ev,re,r,rp) Duel.Hint(HINT_CARD,0,id) Duel.NegateEffect(ev) end
fn negop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // Duel.NegateEffect(ev)
    duel::negate_effect(d, a.ev as u8);
}
// function(e,c) c:UpdateAttack(c:GetMaterial():FilterCount(Card.IsType,nil,TYPE_NORMAL)*1000, RESET_EVENT|RESETS_STANDARD_DISABLE&~RESET_TOFIELD) end
// (the function returns nothing, which the core reads as 0)
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    let mat = card::get_material(d, c);
    let n = group::filter_count(d, &mat, &|d, x| card::is_type(d, x, TYPE_NORMAL), NO_EX) as i64;
    card::update_attack(d, c, n * 1000, Some(RESET_EVENT | (RESETS_STANDARD_DISABLE & !RESET_TOFIELD)), None, None);
    0
}
// function(e) return e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD) end
fn cond3(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, h, LOCATION_ONFIELD)
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsType(TYPE_NORMAL) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    card::is_type(d, c, TYPE_NORMAL) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        let e = a.e;
        let tp = a.tp;
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
        duel::special_summon(d, &g, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None);
    }
}
