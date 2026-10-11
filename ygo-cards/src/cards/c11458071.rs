//! Sky Scourge Enrise
use super::prelude::*;

pub const ID: u32 = 11458071;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Must be properly summoned before reviving
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Must be special summon by its own method
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // e1:SetValue(aux.FALSE)
    effect::set_value_fn(d, e1, aux_false());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special summon procedure (from hand)
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e2, EFFECT_SPSUMMON_PROC);
    // e2:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(lua_fn(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(lua_fn(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(lua_fn(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Banish 1 monster on the field
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e3, CATEGORY_REMOVE);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCost(s.rmcost)
    effect::set_cost(d, e3, Some(cost_target(rmcost)));
    // e3:SetTarget(s.rmtg)
    effect::set_target(d, e3, Some(cost_target(rmtg)));
    // e3:SetOperation(s.rmop)
    effect::set_operation(d, e3, Some(operation(rmop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, mg: &CardSet, c: Option<CardId>) -> (bool, bool) {
    // return aux.ChkfMMZ(1)(sg,e,tp,mg) and sg:IsExists(s.atchk1,1,nil,sg)
    let mmz = aux::chkf_mmz(1)(d, sg, e, tp, mg, c).0;
    (mmz && group::is_exists(d, sg, &|d, x| atchk1(d, x, sg), 1, NO_EX), false)
}
/// `s.atchk1(c,sg)`
fn atchk1(d: &mut Duel, c: CardId, sg: &CardSet) -> bool {
    // return c:IsRace(RACE_FIEND) and c:IsAttribute(ATTRIBUTE_DARK) and sg:FilterCount(s.atchk2,c)==3
    card::is_race(d, c, RACE_FIEND)
        && card::is_attribute(d, c, ATTRIBUTE_DARK)
        && group::filter_count(d, sg, &atchk2, c.into()) == 3
}
/// `s.atchk2(c)`
fn atchk2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_FAIRY) and c:IsAttribute(ATTRIBUTE_LIGHT)
    card::is_race(d, c, RACE_FAIRY) && card::is_attribute(d, c, ATTRIBUTE_LIGHT)
}
/// `s.spfilter(c,rac,att)`
fn spfilter(d: &mut Duel, c: CardId, rac: u64, att: u32) -> bool {
    // return c:IsRace(rac) and c:IsAttribute(att) and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)
    card::is_race(d, c, rac) && card::is_attribute(d, c, att) && card::is_able_to_remove_as_cost(d, c, None) && aux::sp_elim_filter(d, c, true, false)
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local rg1=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil,RACE_FAIRY,ATTRIBUTE_LIGHT)
    let rg1 = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, RACE_FAIRY, ATTRIBUTE_LIGHT)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // local rg2=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil,RACE_FIEND,ATTRIBUTE_DARK)
    let rg2 = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, RACE_FIEND, ATTRIBUTE_DARK)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // local rg=rg1:Clone()
    // rg:Merge(rg2)
    let mut rg = rg1.clone();
    group::add_card(&mut rg, &rg2);
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>-4 and #rg1>2 and #rg2>0
    //     and aux.SelectUnselectGroup(rg,e,tp,4,4,s.rescon,0)
    let count = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    if count <= -4 || rg1.len() <= 2 || rg2.is_empty() {
        return false;
    }
    let rs: aux::Rescon = &rescon;
    matches!(aux::select_unselect_group(d, &rg, Some(e), tp, Some(4), Some(4), Some(rs), 0, tp, None, None, None, false), Err(true))
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,c)`
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local rg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil,RACE_FAIRY,ATTRIBUTE_LIGHT)
    let mut rg = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, RACE_FAIRY, ATTRIBUTE_LIGHT)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // rg:Merge(Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil,RACE_FIEND,ATTRIBUTE_DARK))
    let rg2 = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, RACE_FIEND, ATTRIBUTE_DARK)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    group::add_card(&mut rg, &rg2);
    // local g=aux.SelectUnselectGroup(rg,e,tp,4,4,s.rescon,1,tp,HINTMSG_REMOVE,nil,nil,true)
    let rs: aux::Rescon = &rescon;
    let g = aux::select_unselect_group(d, &rg, Some(e), tp, Some(4), Some(4), Some(rs), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, true).unwrap_or_default();
    // if #g>0 then
    if !g.is_empty() {
        // e:SetLabelObject(g)
        effect::set_label_object(d, e, Value::Group(g));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local g=e:GetLabelObject()
    // if not g then return end
    let Value::Group(g) = effect::get_label_object(d, e) else {
        return;
    };
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    // g:DeleteGroup()
}
/// `s.rmcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmcost(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return e:GetHandler():GetAttackAnnouncedCount()==0 end
    if a.chk == Some(0) {
        return card::get_attack_announced_count(d, h) == 0;
    }
    // Cannot attack this turn
    // local e1=Effect.CreateEffect(e:GetHandler())
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(3206)
    effect::set_description(d, e1, 3206);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_OATH+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_OATH + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_ATTACK)
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // e:GetHandler():RegisterEffect(e1,true)
    card::register_effect(d, h, e1, true);
    true
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsAbleToRemove()
    card::is_faceup(d, c) && card::is_able_to_remove(d, c, None, None, None)
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.tgfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && tgfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&tgfilter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, tp, Some(&tgfilter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, 0, 0);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
            // Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)
            duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
    }
}
