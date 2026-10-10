//! Chaos Emperor, the Dragon of Armageddon
use super::prelude::*;

pub const ID: u32 = 4538826;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // Add 1 banished Dragon monster to the hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(Cost.PayLP(1000))
    effect::set_cost(d, e1, Some(cost::pay_lp(cost::PayLp::Amount(1000))));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon condition
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e2, EFFECT_SPSUMMON_CONDITION);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon procedure
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e3, EFFECT_SPSUMMON_PROC);
    // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e3:SetCountLimit(1,{id,1},EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e3, 1, ID, 1, EFFECT_COUNT_CODE_OATH as u8);
    // e3:SetRange(LOCATION_EXTRA|LOCATION_HAND)
    effect::set_range(d, e3, LOCATION_EXTRA | LOCATION_HAND);
    // e3:SetCondition(s.spcon)
    effect::set_condition(d, e3, Some(lua_fn(spcon)));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(lua_fn(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(lua_fn(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Send cards to the GY and inflict damage
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_TOGRAVE+CATEGORY_DAMAGE)
    effect::set_category(d, e4, CATEGORY_TOGRAVE + CATEGORY_DAMAGE);
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetCost(s.gycost)
    effect::set_cost(d, e4, Some(cost_target(gycost)));
    // e4:SetTarget(s.gytg)
    effect::set_target(d, e4, Some(cost_target(gytg)));
    // e4:SetOperation(s.gyop)
    effect::set_operation(d, e4, Some(operation(gyop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Send itself to the Deck if it leaves the field
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
    // e5:SetCode(EFFECT_LEAVE_FIELD_REDIRECT)
    effect::set_code(d, e5, EFFECT_LEAVE_FIELD_REDIRECT);
    // e5:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e5, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e5:SetCondition(function(e) return e:GetHandler():HasFlagEffect(id) end)
    effect::set_condition(d, e5, Some(lua_fn(redirect_cond)));
    // e5:SetValue(LOCATION_DECKBOT)
    effect::set_value(d, e5, LOCATION_DECKBOT as i64);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e6, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e6:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e6, EVENT_SPSUMMON_SUCCESS);
    // e6:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e6, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e6:SetOperation(function(e) e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1) end)
    effect::set_operation(d, e6, Some(lua_fn(flag_op)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_DRAGON) and c:IsAbleToHand()
    card::is_faceup(d, c) && card::is_race(d, c, RACE_DRAGON) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_REMOVED) and chkc:IsControler(tp) and s.thfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_REMOVED) && card::is_controler(d, chkc, tp) && thfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.thfilter,tp,LOCATION_REMOVED,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&thfilter), tp, LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.thfilter,tp,LOCATION_REMOVED,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&thfilter), tp, LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from(h)), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), 1, tp, 0);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and tc:IsRelateToEffect(e) and Duel.Destroy(c,REASON_EFFECT)~=0 then
    if card::is_relate_to_effect(d, c, a.e) && card::is_relate_to_effect(d, tc.nil(), a.e) && duel::destroy(d, c, REASON_EFFECT, None, None) != 0 {
        // Duel.SendtoHand(tc,nil,REASON_EFFECT)
        duel::send_to_hand(d, tc.nil(), None, REASON_EFFECT, None);
    }
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    let e = e.nil();
    let h = effect::get_handler(d, e).nil();
    // return sg:IsExists(s.atchk1,1,nil,sg)
    //   and ((not e:GetHandler():IsLocation(LOCATION_EXTRA) and aux.ChkfMMZ(1)(sg,e,tp,mg))
    //   or (e:GetHandler():IsLocation(LOCATION_EXTRA) and Duel.GetLocationCountFromEx(tp,tp,sg,e:GetHandler())>0))
    let exists = group::is_exists(d, sg, &|d, c| atchk1(d, c, sg), 1, NO_EX);
    let ok = exists
        && ((!card::is_location(d, h, LOCATION_EXTRA) && aux::chkf_mmz(1)(d, sg, Some(e), tp, mg, None).0)
            || (card::is_location(d, h, LOCATION_EXTRA) && duel::get_location_count_from_ex(d, tp, Some(tp), sg.into(), duel::ExtraCard::Card(h), None).0 > 0));
    (ok, false)
}
/// `s.atchk1(c,sg)`
fn atchk1(d: &mut Duel, c: CardId, sg: &CardSet) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT) and sg:FilterCount(Card.IsAttribute,c,ATTRIBUTE_DARK)==1
    card::is_attribute(d, c, ATTRIBUTE_LIGHT) && group::filter_count(d, sg, &|d, x| card::is_attribute(d, x, ATTRIBUTE_DARK), c.into()) == 1
}
/// `s.spfilter(c,att)`
fn spfilter(d: &mut Duel, c: CardId, att: u32) -> bool {
    // return c:IsAttribute(att) and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)
    card::is_attribute(d, c, att) && card::is_able_to_remove_as_cost(d, c, None) && aux::sp_elim_filter(d, c, true, false)
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
    // local rg1=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil,ATTRIBUTE_LIGHT)
    let rg1 = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, ATTRIBUTE_LIGHT)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // local rg2=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil,ATTRIBUTE_DARK)
    let rg2 = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, ATTRIBUTE_DARK)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // local rg=rg1:Clone()
    let mut rg = rg1.clone();
    // rg:Merge(rg2)
    group::add_card(&mut rg, &rg2);
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>-2 and #rg1>0 and #rg2>0
    //   and aux.SelectUnselectGroup(rg,e,tp,2,2,s.rescon,0)
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > -2
        && !rg1.is_empty()
        && !rg2.is_empty()
        && matches!(aux::select_unselect_group(d, &rg, Some(e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false), Err(true))
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,c)`
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local rg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil,ATTRIBUTE_LIGHT|ATTRIBUTE_DARK)
    let rg = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, ATTRIBUTE_LIGHT | ATTRIBUTE_DARK)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // local g=aux.SelectUnselectGroup(rg,e,tp,2,2,s.rescon,1,tp,HINTMSG_REMOVE,nil,nil,true)
    let g = aux::select_unselect_group(d, &rg, Some(e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, true).unwrap_or_default();
    // if #g>0 then
    if !g.is_empty() {
        // g:KeepAlive()
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
    let g = match effect::get_label_object(d, e) {
        Value::Group(g) => g,
        _ => crate::lua_error("attempt to index a nil value"),
    };
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    // g:DeleteGroup()
}
/// `s.gycost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn gycost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.PayLPCost(tp,math.floor(Duel.GetLP(tp)/2))
    let lp = duel::get_lp(d, a.tp);
    duel::pay_lp_cost(d, a.tp, lp.div_euclid(2) as u32);
    true
}
/// `s.sgfilter(c,p)`
fn sgfilter(d: &mut Duel, c: CardId, p: u8) -> bool {
    // return c:IsLocation(LOCATION_GRAVE) and c:IsControler(p)
    card::is_location(d, c, LOCATION_GRAVE) && card::is_controler(d, c, p)
}
/// `s.gytg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn gytg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(nil,tp,LOCATION_SZONE|LOCATION_MMZONE,0,nil)
    let mut g = duel::get_matching_group(d, None, tp, LOCATION_SZONE | LOCATION_MMZONE, 0, NO_EX);
    // local og=Duel.GetFieldGroup(tp,0,LOCATION_ONFIELD)
    let og = duel::get_field_group(d, tp, 0, LOCATION_ONFIELD);
    // if chk==0 then return #g>0 and #og>0 end
    if a.chk == Some(0) {
        return !g.is_empty() && !og.is_empty();
    }
    // local oc=#og
    let oc = og.len();
    // g:Merge(og)
    group::add_card(&mut g, &og);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(g), n, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,0,0,1-tp,oc*300)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, (oc * 300) as i32);
    true
}
/// `s.gyop(e,tp,eg,ep,ev,re,r,rp)`
fn gyop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(nil,tp,LOCATION_SZONE|LOCATION_MMZONE,0,nil)
    let g = duel::get_matching_group(d, None, tp, LOCATION_SZONE | LOCATION_MMZONE, 0, NO_EX);
    // if #g==0 or Duel.SendtoGrave(g,REASON_EFFECT)==0 then return end
    if g.is_empty() || duel::send_to_grave(d, g, REASON_EFFECT, None, None) == 0 {
        return;
    }
    // local oc=Duel.GetOperatedGroup():FilterCount(Card.IsLocation,nil,LOCATION_GRAVE)
    let og = duel::get_operated_group(d);
    let oc = group::filter_count(d, &og, &|d, c| card::is_location(d, c, LOCATION_GRAVE), NO_EX);
    // if oc==0 then return end
    if oc == 0 {
        return;
    }
    // local dc=Duel.GetOperatedGroup():FilterCount(s.sgfilter,nil,1-tp)
    let og = duel::get_operated_group(d);
    let mut dc = group::filter_count(d, &og, &|d, c| sgfilter(d, c, 1 - tp), NO_EX);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local og=Duel.SelectMatchingCard(tp,nil,tp,0,LOCATION_ONFIELD,1,oc,nil)
    let og = duel::select_matching_card(d, tp, None, tp, 0, LOCATION_ONFIELD, 1, oc as u16, false, NO_EX).nil();
    // if Duel.SendtoGrave(og,REASON_EFFECT)>0 then
    if duel::send_to_grave(d, og, REASON_EFFECT, None, None) > 0 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // dc=dc+Duel.GetOperatedGroup():FilterCount(s.sgfilter,nil,1-tp)
        let og = duel::get_operated_group(d);
        dc += group::filter_count(d, &og, &|d, c| sgfilter(d, c, 1 - tp), NO_EX);
        // Duel.Damage(1-tp,dc*300,REASON_EFFECT)
        duel::damage(d, 1 - tp, (dc * 300) as i64, REASON_EFFECT, false, None);
    }
}
/// `function(e,c) ... e:GetHandler():HasFlagEffect(id) end` (e5's condition)
fn redirect_cond(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0)).nil();
    card::has_flag_effect(d, h, ID, None)
}
/// `function(e) e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1) end` (e6)
fn flag_op(d: &mut Duel, a: Args) {
    let h = effect::get_handler(d, a.e(0)).nil();
    card::register_flag_effect(d, h, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
}
