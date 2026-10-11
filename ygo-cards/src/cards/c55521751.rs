//! Floowandereeze and the Unexplored Winds
//! ふわんだりぃずと未知の風
use super::prelude::*;

pub const ID: u32 = 55521751;

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
    // Tribute Summon by sending 1 of your monsters and 1 opponent's card to the GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_SUMMON_PROC)
    effect::set_code(d, e2, EFFECT_SUMMON_PROC);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(LOCATION_HAND,0)
    effect::set_target_range(d, e2, LOCATION_HAND, 0);
    // e2:SetCondition(s.otcon)
    effect::set_condition(d, e2, Some(lua_fn(otcon)));
    // e2:SetTarget(aux.FieldSummonProcTg(s.ottg,s.sumtg))
    effect::set_target(d, e2, Some(aux::field_summon_proc_tg(Some(lua_fn(ottg)), Some(lua_fn(sumtg)))));
    // e2:SetOperation(s.otop)
    effect::set_operation(d, e2, Some(lua_fn(otop)));
    // e2:SetValue(SUMMON_TYPE_TRIBUTE)
    effect::set_value(d, e2, SUMMON_TYPE_TRIBUTE as i64);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_SET_PROC)
    effect::set_code(d, e3, EFFECT_SET_PROC);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Place up to 2 Winged Beast monsters from your hand on the bottom of your Deck and draw the same number of cards
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_TODECK+CATEGORY_DRAW)
    effect::set_category(d, e4, CATEGORY_TODECK + CATEGORY_DRAW);
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetCountLimit(1,id)
    effect::set_count_limit(d, e4, 1, ID, 0, 0);
    // e4:SetTarget(s.drtg)
    effect::set_target(d, e4, Some(cost_target(drtg)));
    // e4:SetOperation(s.drop)
    effect::set_operation(d, e4, Some(operation(drop_op)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.tgfilter(c,e)`
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsAbleToGrave() and not c:IsImmuneToEffect(e)
    card::is_able_to_grave(d, c, None) && !card::is_immune_to_effect(d, c, e)
}
/// `s.otcon(e,c,minc)`
fn otcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    let minc = a.v(2).int();
    // return minc<=2 and Duel.GetLocationCount(tp,LOCATION_MZONE)>-1
    //     and Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_MZONE,0,1,nil,e)
    //     and Duel.IsExistingMatchingCard(s.tgfilter,tp,0,LOCATION_ONFIELD,1,nil,e)
    minc <= 2
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > -1
        && duel::is_existing_matching_card(d, Some(&|d, cc| tgfilter(d, cc, e)), tp, LOCATION_MZONE, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&|d, cc| tgfilter(d, cc, e)), tp, 0, LOCATION_ONFIELD, 1, NO_EX)
}
/// `s.ottg(e,c)`
fn ottg(d: &mut Duel, a: Args) -> bool {
    // local mi,ma=c:GetTributeRequirement()
    let c = a.c(1);
    let (mi, ma) = card::get_tribute_requirement(d, c);
    // return mi<=2 and ma>=2
    mi <= 2 && ma >= 2
}
/// `s.sumtg(e,tp,eg,ep,ev,re,r,rp,c)`
fn sumtg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.v(1).int() as u8;
    // local mg1=Duel.GetMatchingGroup(s.tgfilter,tp,LOCATION_MZONE,0,nil,e)
    let mg1 = duel::get_matching_group(d, Some(&|d, c| tgfilter(d, c, e)), tp, LOCATION_MZONE, 0, NO_EX);
    // local mg2=Duel.GetMatchingGroup(s.tgfilter,tp,0,LOCATION_ONFIELD,nil,e)
    let mg2 = duel::get_matching_group(d, Some(&|d, c| tgfilter(d, c, e)), tp, 0, LOCATION_ONFIELD, NO_EX);
    // ::restart::
    loop {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
        // local g1=mg1:Select(tp,1,1,true,nil)
        let Some(g1) = group::select(d, &mg1, tp, 1, 1, true, NO_EX) else {
            // if not g1 then return false end
            return false;
        };
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
        // local tc=mg2:SelectUnselect(g1,tp,false,false,2,2)
        let tc = group::select_unselect(d, &mg2, Some(&g1), tp, false, false, Some(2), Some(2));
        // if mg2:IsContains(tc) then
        if let Some(tc) = tc.filter(|tc| group::is_contains(&mg2, *tc)) {
            let mut g1 = g1;
            // g1:AddCard(tc)
            group::add_card(&mut g1, &CardSet::from_card(tc));
            // g1:KeepAlive()
            // e:SetLabelObject(g1)
            effect::set_label_object(d, e, Value::Group(g1));
            // return true
            return true;
        }
        // goto restart
    }
}
/// `s.otop(e,tp,eg,ep,ev,re,r,rp,c)`
fn otop(d: &mut Duel, a: Args) {
    // local sg=e:GetLabelObject()
    let sg = effect::get_label_object(d, a.e(0)).as_group().cloned();
    // if not sg then return end
    let Some(sg) = sg else {
        return;
    };
    // Duel.SendtoGrave(sg,REASON_EFFECT)
    duel::send_to_grave(d, sg, REASON_EFFECT, None, None);
    // sg:DeleteGroup()
}
/// `s.drfilter(c)`
fn drfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_WINGEDBEAST) and not c:IsPublic() and c:IsAbleToDeck()
    card::is_race(d, c, RACE_WINGEDBEAST as u64) && !card::is_public(d, c) && card::is_able_to_deck(d, c)
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1) and Duel.IsExistingMatchingCard(s.drfilter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, tp, 1) && duel::is_existing_matching_card(d, Some(&drfilter), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, tp, LOCATION_HAND as i32);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    true
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectMatchingCard(tp,s.drfilter,tp,LOCATION_HAND,0,1,2,nil)
    let g = duel::select_matching_card(d, tp, Some(&drfilter), tp, LOCATION_HAND, 0, 1, 2, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g.clone());
        // local ct=Duel.SendtoDeck(g,nil,SEQ_DECKBOTTOM,REASON_EFFECT)
        let ct = duel::send_to_deck(d, g.clone(), None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None);
        // if ct>1 then Duel.SortDeckbottom(tp,tp,ct) end
        if ct > 1 {
            duel::sort_deckbottom(d, tp, tp, ct as u16);
        }
        // if ct==#g then
        if ct as usize == g.len() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Draw(tp,ct,REASON_EFFECT)
            duel::draw(d, tp, ct as u32, REASON_EFFECT);
        }
    }
}
