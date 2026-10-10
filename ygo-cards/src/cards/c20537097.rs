//! Mekk-Knight Blue Sky
use super::prelude::*;

pub const ID: u32 = 20537097;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[20537097], listed_series: &[SET_MEKK_KNIGHT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If 2 or more cards are in the same column, you can Special Summon this card (from your hand) in that column
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetValue(s.spval)
    effect::set_value_fn(d, e1, lua_fn(spval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add "Mekk-Knight" monsters with different names, except "Mekk-Knight Blue Sky", from your Deck to your hand,
    // equal to the number of your opponent's cards in this card's column
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2a, aux::stringid(ID, 1));
    // e2a:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2a, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2a:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2a, EFFECT_FLAG_DELAY, 0);
    // e2a:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2a, EVENT_SUMMON_SUCCESS);
    // e2a:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2a, 1, ID, 1, 0);
    // e2a:SetCondition(function(e) return e:GetHandler():IsSummonLocation(LOCATION_HAND) end)
    effect::set_condition(d, e2a, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_summon_location(d, h, LOCATION_HAND)
    })));
    // e2a:SetTarget(s.thtg)
    effect::set_target(d, e2a, Some(cost_target(thtg)));
    // e2a:SetOperation(s.thop)
    effect::set_operation(d, e2a, Some(operation(thop)));
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // local e2b=e2a:Clone()
    let e2b = effect::clone(d, e2a);
    // e2b:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2b, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2b)
    card::register_effect(d, c, e2b, false);
}
/// `s.spval(e,c)`: returns `0,zone`
fn spval(d: &mut Duel, a: Args) -> Value {
    let c = a.c(1);
    // local zone=0
    let mut zone: u32 = 0;
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local g=Duel.GetMatchingGroup(function(c) return c:GetColumnGroupCount()>0 end,0,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&|d, x| card::get_column_group_count(d, x, 0, 0) > 0), 0, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // for tc in g:Iter() do
    for tc in group::iter(d, &g) {
        // zone=(zone|tc:GetColumnZone(LOCATION_MZONE,0,0,tp))
        zone |= card::get_column_zone(d, tc, LOCATION_MZONE, 0, 0, Some(tp));
    }
    // return 0,zone
    Value::Multi(vec![Value::Int(0), Value::Int(zone as i64)])
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MEKK_KNIGHT) and c:IsMonster() and not c:IsCode(id) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_MEKK_KNIGHT as u16]) && card::is_monster(d, c) && !card::is_code(d, c, &[ID]) && card::is_able_to_hand(d, c, None)
}
/// the first code of a card, `Card.GetCode` as the value of `GetClassCount`
fn code_of(d: &mut Duel, c: CardId) -> i64 {
    card::get_code(d, c).first().copied().unwrap_or(0) as i64
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local ct=e:GetHandler():GetColumnGroup():FilterCount(Card.IsControler,nil,1-tp)
    let h = effect::get_handler(d, a.e).nil();
    let column = card::get_column_group(d, h, 0, 0);
    let ct = group::filter_count(d, &column, &|d, x| card::is_controler(d, x, 1 - a.tp), NO_EX);
    // if chk==0 then return ct>0 and Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil):GetClassCount(Card.GetCode)>=ct end
    if a.chk == Some(0) {
        let g = duel::get_matching_group(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, NO_EX);
        return ct > 0 && group::get_class_count(d, &g, &code_of) >= ct;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,ct,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, ct as u8, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local ct=c:GetColumnGroup():FilterCount(Card.IsControler,nil,1-tp)
    let column = card::get_column_group(d, c, 0, 0);
    let mut ct = group::filter_count(d, &column, &|d, x| card::is_controler(d, x, 1 - a.tp), NO_EX);
    // if c:IsControler(1-tp) then ct=ct+1 end
    if card::is_controler(d, c, 1 - a.tp) {
        ct += 1;
    }
    // if ct==0 or g:GetClassCount(Card.GetCode)<ct then return end
    if ct == 0 || group::get_class_count(d, &g, &code_of) < ct {
        return;
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,ct,ct,aux.dncheck,1,tp,HINTMSG_ATOHAND)
    let dn = aux::dncheck();
    let sg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(ct), Some(ct), Some(&dn), 1, a.tp, Some(HINTMSG_ATOHAND as u64), None, None, false).unwrap_or_default();
    // if #sg>0 then
    if !sg.is_empty() {
        // Duel.SendtoHand(sg,nil,REASON_EFFECT)
        duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,sg)
        duel::confirm_cards(d, 1 - a.tp, sg);
    }
}
