//! Salamangreat Sanctuary
use super::prelude::*;

pub const ID: u32 = 1295111;
pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_SALAMANGREAT],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Reincarnation Link Summon effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e2, EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e2:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e2, EFFECT_SPSUMMON_PROC);
    // e2:SetRange(LOCATION_EXTRA)
    effect::set_range(d, e2, LOCATION_EXTRA);
    // e2:SetCondition(s.reinclinkcon)
    effect::set_condition(d, e2, Some(lua_fn(reinclinkcon)));
    // e2:SetTarget(s.reinclinktg)
    effect::set_target(d, e2, Some(lua_fn(reinclinktg)));
    // e2:SetOperation(s.reinclinkop)
    effect::set_operation(d, e2, Some(lua_fn(reinclinkop)));
    // e2:SetValue(SUMMON_TYPE_LINK)
    effect::set_value(d, e2, SUMMON_TYPE_LINK as i64);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_GRANT)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_GRANT);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetTargetRange(LOCATION_EXTRA,0)
    effect::set_target_range(d, e3, LOCATION_EXTRA, 0);
    // e3:SetTarget(function(e,c) return c:IsSetCard(SET_SALAMANGREAT) and c:IsLinkMonster() end)
    effect::set_target(d, e3, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_SALAMANGREAT as u16]) && card::is_link_monster(d, c))));
    // e3:SetLabelObject(e2)
    effect::set_label_object(d, e3, Value::Effect(e2));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Gain LP equal to the original ATK of a Link Monster you control
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_RECOVER)
    effect::set_category(d, e4, CATEGORY_ATKCHANGE + CATEGORY_RECOVER);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e4:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET, 0);
    // e4:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e4, EVENT_PRE_DAMAGE_CALCULATE);
    // e4:SetRange(LOCATION_FZONE)
    effect::set_range(d, e4, LOCATION_FZONE);
    // e4:SetCountLimit(1,id)
    effect::set_count_limit(d, e4, 1, ID, 0, 0);
    // e4:SetCondition(function(e,tp) return Duel.GetBattleMonster(tp) end)
    effect::set_condition(d, e4, Some(condition(|d, a| duel::get_battle_monster(d, a.tp).0.is_some())));
    // e4:SetCost(Cost.PayLP(1000))
    effect::set_cost(d, e4, Some(cost::pay_lp(cost::PayLp::Amount(1000))));
    // e4:SetTarget(s.atktg)
    effect::set_target(d, e4, Some(cost_target(atktg)));
    // e4:SetOperation(s.atkop)
    effect::set_operation(d, e4, Some(operation(atkop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
// function s.reincmatfilter(c,lc,tp)
fn reincmatfilter(d: &mut Duel, c: CardId, lc: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsLinkMonster()
    //   and c:IsSummonCode(lc,SUMMON_TYPE_LINK,tp,lc:GetCode()) and c:IsCanBeLinkMaterial(lc,tp)
    //   and Duel.GetLocationCountFromEx(tp,tp,c,lc)>0
    let lc_codes = card::get_code(d, lc);
    card::is_faceup(d, c)
        && card::is_link_monster(d, c)
        && card::is_summon_code(d, c, Some(lc), SUMMON_TYPE_LINK as u64, tp, &lc_codes)
        && card::is_can_be_link_material(d, c, Some(lc), Some(tp))
        && duel::get_location_count_from_ex(d, tp, Some(tp), c.into(), duel::ExtraCard::Card(lc), None).0 > 0
}
// function s.reinclinkcon(e,c,must,g,min,max)
fn reinclinkcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local g=Duel.GetMatchingGroup(s.reincmatfilter,tp,LOCATION_MZONE,0,nil,c,tp)
    let g = duel::get_matching_group(d, Some(&|d, cc| reincmatfilter(d, cc, c, tp)), tp, LOCATION_MZONE, 0, NO_EX);
    // local mustg=Auxiliary.GetMustBeMaterialGroup(tp,g,tp,c,g,REASON_LINK)
    let mut mustg = aux::get_must_be_material_group(d, tp, &g, tp, Some(c), &g, REASON_LINK as u32);
    // if must then mustg:Merge(must) end
    if let Some(must) = a.v(2).as_group().cloned() {
        group::add_card(&mut mustg, &must);
    }
    // return ((#mustg==1 and s.reincmatfilter(mustg:GetFirst(),c,tp)) or (#mustg==0 and #g>0))
    //   and not Duel.HasFlagEffect(tp,id)
    ((mustg.len() == 1 && reincmatfilter(d, group::get_first(d, &mustg).nil(), c, tp)) || (mustg.is_empty() && !g.is_empty())) && !duel::has_flag_effect(d, tp, ID, None)
}
// function s.reinclinktg(e,tp,eg,ep,ev,re,r,rp,chk,c,must,g,min,max)
fn reinclinktg(d: &mut Duel, a: Args) -> bool {
    let (e, tp, c) = (a.e(0), a.p(1), a.c(9));
    // local g=Duel.GetMatchingGroup(s.reincmatfilter,tp,LOCATION_MZONE,0,nil,c,tp)
    let g = duel::get_matching_group(d, Some(&|d, cc| reincmatfilter(d, cc, c, tp)), tp, LOCATION_MZONE, 0, NO_EX);
    // local mustg=Auxiliary.GetMustBeMaterialGroup(tp,g,tp,c,g,REASON_LINK)
    let mut mustg = aux::get_must_be_material_group(d, tp, &g, tp, Some(c), &g, REASON_LINK as u32);
    // if must then mustg:Merge(must) end
    if let Some(must) = a.v(10).as_group().cloned() {
        group::add_card(&mut mustg, &must);
    }
    // if #mustg>0 then
    if !mustg.is_empty() {
        // if #mustg>1 then return false end
        if mustg.len() > 1 {
            return false;
        }
        // e:SetLabelObject(mustg)
        effect::set_label_object(d, e, Value::Group(mustg));
        // return true
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_LMATERIAL)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_LMATERIAL as u64);
    // local tc=g:SelectUnselect(Group.CreateGroup(),tp,false,true)
    let tc = group::select_unselect(d, &g, Some(&CardSet::new()), tp, false, true, None, None);
    // if tc then
    if let Some(tc) = tc {
        // local sg=Group.FromCards(tc)
        // e:SetLabelObject(sg)
        effect::set_label_object(d, e, Value::Group(CardSet::from_card(tc)));
        // return true
        true
    } else {
        // else return false end
        false
    }
}
// function s.reinclinkop(e,tp,eg,ep,ev,re,r,rp,c,must,g,min,max)
fn reinclinkop(d: &mut Duel, a: Args) -> Value {
    let (e, tp, c) = (a.e(0), a.p(1), a.c(8));
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // local mg=e:GetLabelObject()
    let mg = effect::get_label_object(d, e).as_group().cloned().nil();
    // c:SetMaterial(mg)
    card::set_material(d, c, mg.clone());
    // Duel.SendtoGrave(mg,REASON_MATERIAL|REASON_LINK)
    duel::send_to_grave(d, mg, REASON_MATERIAL | REASON_LINK, None, None);
    // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    Value::Nil
}
// function s.atkfilter(c)
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsLinkMonster() and not c:IsAttack(0)
    card::is_faceup(d, c) && card::is_link_monster(d, c) && !card::is_attack(d, c, &[0])
}
// function s.atktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.atkfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && atkfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.atkfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&atkfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATKDEF)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATKDEF as u64);
    // local g=Duel.SelectTarget(tp,s.atkfilter,tp,LOCATION_MZONE,0,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&atkfilter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // local rec=g:GetFirst():GetBaseAttack()
    let tc = group::get_first(d, &g).nil();
    let rec = card::get_base_attack(d, tc);
    // Duel.SetTargetParam(rec)
    duel::set_target_param(d, rec as u32);
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,rec)
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, rec);
    true
}
// function s.atkop(e,tp,eg,ep,ev,re,r,rp)
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() and not tc:IsAttack(0) then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) && !card::is_attack(d, tc, &[0]) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetValue(0)
        effect::set_value(d, e1, 0);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // if not tc:IsAttack(0) then return end
        if !card::is_attack(d, tc, &[0]) {
            return;
        }
        // Duel.Recover(tp,tc:GetBaseAttack(),REASON_EFFECT)
        let base = card::get_base_attack(d, tc);
        duel::recover(d, a.tp, base as i64, REASON_EFFECT, false, None);
    }
}
