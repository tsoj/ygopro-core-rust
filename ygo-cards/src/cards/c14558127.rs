//! Ash Blossom & Joyous Spring
use super::prelude::*;

pub const ID: u32 = 14558127;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Negate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DISABLE);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(discon)));
    effect::set_cost(d, e1, Some(cost::self_discard()));
    effect::set_target(d, e1, Some(cost_target(distg)));
    effect::set_operation(d, e1, Some(operation(disop)));
    card::register_effect(d, c, e1, false);
}
/// `s.check(ev,re)`: returns `function(category,checkloc)`.
fn check(ev: u32, re: EffectId) -> impl Fn(&mut Duel, u64, bool) -> bool {
    move |d, category, checkloc| {
        // if not checkloc and re:IsHasCategory(category) then return true end
        if !checkloc && effect::is_has_category(d, re, category) {
            return true;
        }
        // local ex1,g1,gc1,dp1,dv1=Duel.GetOperationInfo(ev,category)
        let info1 = duel::get_operation_info(d, ev, category);
        // local ex2,g2,gc2,dp2,dv2=Duel.GetPossibleOperationInfo(ev,category)
        let info2 = duel::get_possible_operation_info(d, ev, category);
        // if not (ex1 or ex2) then return false end
        if info1.is_none() && info2.is_none() {
            return false;
        }
        // if category==CATEGORY_DRAW or category==CATEGORY_DECKDES then return true end
        if category == CATEGORY_DRAW || category == CATEGORY_DECKDES {
            return true;
        }
        // local g=Group.CreateGroup()
        let mut g = group::create_group(&[]);
        // if g1 then g:Merge(g1) end
        if let Some(g1) = info1.as_ref().and_then(|i| i.op_cards.as_ref()) {
            group::add_card(&mut g, g1);
        }
        // if g2 then g:Merge(g2) end
        if let Some(g2) = info2.as_ref().and_then(|i| i.op_cards.as_ref()) {
            group::add_card(&mut g, g2);
        }
        // return (((dv1 or 0)|(dv2 or 0))&LOCATION_DECK)~=0 or (#g>0 and g:IsExists(Card.IsLocation,1,nil,LOCATION_DECK))
        let dv1 = info1.as_ref().map_or(0, |i| i.op_param);
        let dv2 = info2.as_ref().map_or(0, |i| i.op_param);
        ((dv1 | dv2) & LOCATION_DECK as i32) != 0
            || (!g.is_empty() && group::is_exists(d, &g, &|d, c| card::is_location(d, c, LOCATION_DECK), 1, NO_EX))
    }
}
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // if re:GetHandler():IsDisabled() or not Duel.IsChainDisablable(ev) then return false end
    let re = a.re.nil();
    if card::is_disabled(d, effect::get_handler(d, re).nil()) || !duel::is_chain_disablable(d, a.ev as u8) {
        return false;
    }
    // local checkfunc=s.check(ev,re)
    let checkfunc = check(a.ev, re);
    // return checkfunc(CATEGORY_TOHAND,true) or checkfunc(CATEGORY_SPECIAL_SUMMON,true)
    //     or checkfunc(CATEGORY_TOGRAVE,true) or checkfunc(CATEGORY_DRAW,true) or checkfunc(CATEGORY_DRAW,false)
    //     or checkfunc(CATEGORY_SEARCH,false) or checkfunc(CATEGORY_DECKDES,true) or checkfunc(CATEGORY_DECKDES,false)
    checkfunc(d, CATEGORY_TOHAND, true)
        || checkfunc(d, CATEGORY_SPECIAL_SUMMON, true)
        || checkfunc(d, CATEGORY_TOGRAVE, true)
        || checkfunc(d, CATEGORY_DRAW, true)
        || checkfunc(d, CATEGORY_DRAW, false)
        || checkfunc(d, CATEGORY_SEARCH, false)
        || checkfunc(d, CATEGORY_DECKDES, true)
        || checkfunc(d, CATEGORY_DECKDES, false)
}
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return not re:GetHandler():IsStatus(STATUS_DISABLED) end
    if a.chk == Some(0) {
        return !card::is_status(d, effect::get_handler(d, a.re.nil()).nil(), STATUS_DISABLED);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, 0, 0);
    true
}
fn disop(d: &mut Duel, a: &EffArgs) {
    // Duel.NegateEffect(ev)
    duel::negate_effect(d, a.ev as u8);
}
