//! Red-Eyes Slash Dragon
use super::prelude::*;

pub const ID: u32 = 21140872;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.listed_series={SET_RED_EYES}
    listed_series: &[SET_RED_EYES],
    // s.listed_names={CARD_NECROVALLEY}
    listed_names: &[CARD_NECROVALLEY],
    // s.material_setcode=SET_RED_EYES
    material_setcode: &[SET_RED_EYES],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcCodeFun(c,CARD_REDEYES_B_DRAGON,aux.FilterBoolFunction(Card.IsRace,RACE_WARRIOR),1,true)
    fusion::add_proc_code_fun(
        d,
        c,
        CARD_REDEYES_B_DRAGON,
        fusion::fus_mat(|d, x, fc, st, tp| card::is_race_ctx(d, x, RACE_WARRIOR, Some(fc), st, Some(tp))),
        1,
        true,
        false,
    );
    // Equip
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.eqcon)
    effect::set_condition(d, e1, Some(condition(eqcon)));
    // e1:SetTarget(s.eqtg)
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    // e1:SetOperation(s.eqop)
    effect::set_operation(d, e1, Some(operation(eqop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Negate
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_NEGATE+CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_NEGATE + CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e2:SetCondition(s.ngcon)
    effect::set_condition(d, e2, Some(condition(ngcon)));
    // e2:SetCost(s.ngcost)
    effect::set_cost(d, e2, Some(cost_target(ngcost)));
    // e2:SetTarget(s.ngtg)
    effect::set_target(d, e2, Some(cost_target(ngtg)));
    // e2:SetOperation(s.ngop)
    effect::set_operation(d, e2, Some(operation(ngop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // special summon
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e4:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e4, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e4:SetCode(EVENT_LEAVE_FIELD_P)
    effect::set_code(d, e4, EVENT_LEAVE_FIELD_P);
    // e4:SetOperation(s.eqcheck)
    effect::set_operation(d, e4, Some(operation(eqcheck)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e3, EVENT_DESTROYED);
    // e3:SetCondition(s.spcon2)
    effect::set_condition(d, e3, Some(condition(spcon2)));
    // e3:SetTarget(s.sptg2)
    effect::set_target(d, e3, Some(cost_target(sptg2)));
    // e3:SetOperation(s.spop2)
    effect::set_operation(d, e3, Some(operation(spop2)));
    // e3:SetLabelObject(e4)
    effect::set_label_object(d, e3, Value::Effect(e4));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.eqcon(e)`
fn eqcon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.GetAttacker():IsSetCard(SET_RED_EYES)
    let at = duel::get_attacker(d).nil();
    card::is_set_card(d, at, &[SET_RED_EYES as u16])
}
/// `s.eqfilter(c,tp)`
fn eqfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsRace(RACE_WARRIOR) and c:CheckUniqueOnField(tp) and c:IsMonster() and not c:IsForbidden()
    card::is_race(d, c, RACE_WARRIOR) && card::check_unique_on_field(d, c, tp, None, None) && card::is_monster(d, c) && !card::is_forbidden(d, c)
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.eqfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && eqfilter(d, chkc, tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.eqfilter,tp,LOCATION_GRAVE,0,1,nil,tp)
    //     and Duel.GetLocationCount(tp,LOCATION_SZONE)>0 end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| eqfilter(d, c, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
            && duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    // local g=Duel.SelectTarget(tp,s.eqfilter,tp,LOCATION_GRAVE,0,1,1,nil,tp)
    let g = duel::select_target(d, tp, Some(&|d, c| eqfilter(d, c, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, g.clone(), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, g, 1, 0, 0);
    true
}
/// `s.eqlimit(e,c)`: `return c==e:GetLabelObject()`
fn eqlimit(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let lo = effect::get_label_object(d, e).as_card();
    Some(a.c(1)) == lo
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.Equip(tp,tc,c,true) then
    if card::is_relate_to_effect(d, tc, e) && duel::equip(d, tp, tc, c, true, false) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_EQUIP_LIMIT)
        effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // e1:SetValue(s.eqlimit)
        effect::set_value_fn(d, e1, lua_fn(eqlimit));
        // e1:SetLabelObject(c)
        effect::set_label_object(d, e1, Value::Card(c));
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // --atkup
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_EQUIP)
        effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
        // e2:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
        // e2:SetValue(200)
        effect::set_value(d, e2, 200);
        // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
    }
}
/// `s.ngcfilter(c,tp)`
fn ngcfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsControler(tp) and c:IsOnField()
    card::is_controler(d, c, tp) && card::is_on_field(d, c)
}
/// `s.ngcon(e,tp,eg,ep,ev,re,r,rp)`
fn ngcon(d: &mut Duel, a: &EffArgs) -> bool {
    // if not re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) then return false end
    let re = a.re.nil();
    if !effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) {
        return false;
    }
    // local g=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]).first().and_then(|v| v.as_group().cloned());
    // return g and g:IsExists(s.ngcfilter,1,nil,tp) and Duel.IsChainNegatable(ev)
    match g {
        Some(g) => group::is_exists(d, &g, &|d, c| ngcfilter(d, c, a.tp), 1, NO_EX) && duel::is_chain_negatable(d, a.ev as u8),
        None => false,
    }
}
/// `s.ngfilter(c,e,tp)`
fn ngfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_EQUIP) and c:IsAbleToGraveAsCost()
    card::is_type(d, c, TYPE_EQUIP) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.ngcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn ngcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.ngfilter,tp,LOCATION_SZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&ngfilter), tp, LOCATION_SZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.ngfilter,tp,LOCATION_SZONE,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&ngfilter), tp, LOCATION_SZONE, 0, 1, 1, false, NO_EX);
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g.unwrap_or_default(), REASON_COST, None, None);
    true
}
/// `s.ngtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn ngtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    // if re:GetHandler():IsDestructable() and re:GetHandler():IsRelateToEffect(re) then
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    if card::is_destructable(d, rc, None) && card::is_relate_to_effect(d, rc, re) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}
/// `s.ngop(e,tp,eg,ep,ev,re,r,rp)`
fn ngop(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateActivation(ev) then
    if duel::negate_activation(d, a.ev as u8) {
        // Duel.Destroy(eg,REASON_EFFECT)
        duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
    }
}
/// `s.eqcheck(e,tp,eg,ep,ev,re,r,rp)`
fn eqcheck(d: &mut Duel, a: &EffArgs) {
    // if e:GetLabelObject() then e:GetLabelObject():DeleteGroup() end
    // (DeleteGroup and KeepAlive do nothing here: groups are values)
    // local g=e:GetHandler():GetEquipGroup()
    let h = effect::get_handler(d, a.e).nil();
    let g = card::get_equip_group(d, h);
    // e:SetLabelObject(g)
    effect::set_label_object(d, a.e, Value::Group(g));
}
/// `s.spcon2(e,tp,eg,ep,ev,re,r,rp)`
fn spcon2(_d: &mut Duel, a: &EffArgs) -> bool {
    // return (r&REASON_EFFECT+REASON_BATTLE)~=0
    (a.r & (REASON_EFFECT + REASON_BATTLE)) != 0
}
/// `s.spfilter2(c,e,tp)`
fn spfilter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLocation(LOCATION_GRAVE) and c:IsControler(tp) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_location(d, c, LOCATION_GRAVE) && card::is_controler(d, c, tp) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg2(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local g=e:GetLabelObject():GetLabelObject()
    let e4 = effect::get_label_object(d, e).as_effect().nil();
    let g = effect::get_label_object(d, e4).as_group().cloned();
    // if chk==0 then return g and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and g:IsExists(s.spfilter2,1,nil,e,tp) and not Duel.IsEnvironment(CARD_NECROVALLEY) end
    if a.chk == Some(0) {
        return match &g {
            Some(g) => {
                duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
                    && group::is_exists(d, g, &|d, c| spfilter2(d, c, e, tp), 1, NO_EX)
                    && !duel::is_environment(d, CARD_NECROVALLEY, None, None)
            }
            None => false,
        };
    }
    // local sg=g:Filter(s.spfilter2,nil,e,tp)
    let g = g.nil();
    let sg = group::filter(d, &g, &|d, c| spfilter2(d, c, e, tp), NO_EX);
    // Duel.SetTargetCard(sg)
    duel::set_target_card(d, sg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,sg,#sg,0,0)
    let n = sg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(sg), n, 0, 0);
    true
}
/// `s.spop2(e,tp,eg,ep,ev,re,r,rp)`
fn spop2(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local sg=Duel.GetTargetCards(e)
    let mut sg = duel::get_target_cards(d, a.e).unwrap_or_default();
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if ft<=0 then return end
    if ft <= 0 {
        return;
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    // if #sg>ft then
    if sg.len() as i32 > ft {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // sg=sg:Select(tp,ft,ft,nil)
        sg = group::select(d, &sg, tp, ft as u16, ft as u16, false, NO_EX).nil();
    }
    // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
    duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
}
