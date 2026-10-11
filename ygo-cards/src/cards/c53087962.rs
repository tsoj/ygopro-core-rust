//! Ursarctic Septentrion
use super::prelude::*;

pub const ID: u32 = 53087962;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_URSARCTIC], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // --Must be special summoned by its own method
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Special summon procedure (from extra deck)
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e2, EFFECT_SPSUMMON_PROC);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetRange(LOCATION_EXTRA)
    effect::set_range(d, e2, LOCATION_EXTRA);
    // e2:SetCondition(s.sprcon)
    effect::set_condition(d, e2, Some(lua_fn(|d, a: Args| sprcon(d, a.e(0), a.card(1)))));
    // e2:SetTarget(s.sprtg)
    effect::set_target(d, e2, Some(lua_fn(|d, a: Args| sprtg(d, a.e(0), a.p(1)))));
    // e2:SetOperation(s.sprop)
    effect::set_operation(d, e2, Some(operation(sprop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // --disable
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_DISABLE)
    effect::set_code(d, e3, EFFECT_DISABLE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e3, LOCATION_MZONE, LOCATION_MZONE);
    // e3:SetTarget(s.distg)
    effect::set_target(d, e3, Some(effect_card_fn(distg)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // --search
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e4, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e4:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e4, EFFECT_FLAG_DELAY, 0);
    // e4:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    // e4:SetCountLimit(1,id)
    effect::set_count_limit(d, e4, 1, ID, 0, 0);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCondition(s.thcon)
    effect::set_condition(d, e4, Some(condition(thcon)));
    // e4:SetTarget(s.thtg)
    effect::set_target(d, e4, Some(cost_target(thtg)));
    // e4:SetOperation(s.thop)
    effect::set_operation(d, e4, Some(operation(thop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.sprfilter(c)`
fn sprfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsAbleToGraveAsCost() and c:HasLevel()
    card::is_faceup(d, c) && card::is_able_to_grave_as_cost(d, c) && card::has_level(d, c)
}
/// `s.sprfilter1(c,tp,g,sc)` (the Lua `g` argument is shadowed inside and never read)
fn sprfilter1(d: &mut Duel, c: CardId, tp: u8, sc: CardId) -> bool {
    // local lv=c:GetLevel()
    let _lv = card::get_level(d, c);
    // local g=Duel.GetMatchingGroup(s.sprfilter,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&sprfilter), tp, LOCATION_MZONE, 0, NO_EX);
    // return c:IsType(TYPE_TUNER) and c:IsLevelAbove(8) and g:IsExists(s.sprfilter2,1,c,tp,c,sc)
    card::is_type(d, c, TYPE_TUNER) && card::is_level_above(d, c, 8) && group::is_exists(d, &g, &|d, x| sprfilter2(d, x, tp, c, sc), 1, c.into())
}
/// `s.sprfilter2(c,tp,mc,sc)`
fn sprfilter2(d: &mut Duel, c: CardId, tp: u8, mc: CardId, sc: CardId) -> bool {
    // local sg=Group.FromCards(c,mc)
    let mut sg = CardSet::from_card(c);
    group::add_card(&mut sg, &CardSet::from_card(mc));
    // return (math.abs((c:GetLevel()-mc:GetLevel()))==7) and c:IsType(TYPE_SYNCHRO) and not c:IsType(TYPE_TUNER)
    //   and Duel.GetLocationCountFromEx(tp,tp,sg,sc)>0
    let lc = card::get_level(d, c) as i64;
    let lm = card::get_level(d, mc) as i64;
    (lc - lm).abs() == 7
        && card::is_type(d, c, TYPE_SYNCHRO)
        && !card::is_type(d, c, TYPE_TUNER)
        && duel::get_location_count_from_ex(d, tp, Some(tp), (&sg).into(), duel::ExtraCard::Card(sc), None).0 > 0
}
/// `s.sprcon(e,c)`
fn sprcon(d: &mut Duel, _e: EffectId, c: Option<CardId>) -> bool {
    // if c==nil then return true end
    let Some(c) = c else { return true };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local g=Duel.GetMatchingGroup(s.sprfilter,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&sprfilter), tp, LOCATION_MZONE, 0, NO_EX);
    // return g:IsExists(s.sprfilter1,1,nil,tp,g,c)
    group::is_exists(d, &g, &|d, x| sprfilter1(d, x, tp, c), 1, NO_EX)
}
/// `s.sprtg(e,tp,eg,ep,ev,re,r,rp,c)`
fn sprtg(d: &mut Duel, e: EffectId, tp: u8) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local g=Duel.GetMatchingGroup(s.sprfilter,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&sprfilter), tp, LOCATION_MZONE, 0, NO_EX);
    // local g1=g:Filter(s.sprfilter1,nil,tp,g,c)
    let g1 = group::filter(d, &g, &|d, x| sprfilter1(d, x, tp, c), NO_EX);
    // local mg1=aux.SelectUnselectGroup(g1,e,tp,1,1,nil,1,tp,HINTMSG_TOGRAVE,nil,nil,true)
    let mut mg1 = aux::select_unselect_group(d, &g1, Some(e), tp, Some(1), Some(1), None, 1, tp, Some(HINTMSG_TOGRAVE as u64), None, None, true).unwrap_or_default();
    // if #mg1>0 then
    if mg1.len() > 0 {
        // local mc=mg1:GetFirst()
        let mc = group::get_first(d, &mg1).nil();
        // mc:GetLevel() is the last argument of the Lua call below (its value is unused)
        let _ = card::get_level(d, mc);
        // local g2=g:Filter(s.sprfilter2,mc,tp,mc,c,mc:GetLevel())
        let g2 = group::filter(d, &g, &|d, x| sprfilter2(d, x, tp, mc, c), mc.into());
        // local mg2=aux.SelectUnselectGroup(g2,e,tp,1,1,nil,1,tp,HINTMSG_TOGRAVE,nil,nil,true)
        let mg2 = aux::select_unselect_group(d, &g2, Some(e), tp, Some(1), Some(1), None, 1, tp, Some(HINTMSG_TOGRAVE as u64), None, None, true).unwrap_or_default();
        // mg1:Merge(mg2)
        group::add_card(&mut mg1, &mg2);
    }
    // if #mg1==2 then
    if mg1.len() == 2 {
        // mg1:KeepAlive()
        // e:SetLabelObject(mg1)
        effect::set_label_object(d, e, Value::Group(mg1));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.sprop(e,tp,eg,ep,ev,re,r,rp,c)`
fn sprop(d: &mut Duel, a: &EffArgs) {
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e).as_group().cloned();
    // if not g then return end
    let Some(g) = g else { return };
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
}
/// `s.distg(e,c)`
fn distg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsSummonLocation(LOCATION_EXTRA) and not c:HasLevel()
    card::is_summon_location(d, c, LOCATION_EXTRA) && !card::has_level(d, c)
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(Card.IsSummonPlayer,1,nil,1-tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| card::is_summon_player(d, c, 1 - a.tp), 1, NO_EX)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_URSARCTIC) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_URSARCTIC as u16]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if g.len() > 0 {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
