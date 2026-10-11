//! Sacred Serpent's Wake
use super::prelude::*;

pub const ID: u32 = 98850929;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 4));
    effect::set_category(d, e1, CATEGORY_TOHAND);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_FUSION+TYPE_RITUAL+TYPE_SYNCHRO+TYPE_XYZ)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_FUSION | TYPE_RITUAL | TYPE_SYNCHRO | TYPE_XYZ)
}
fn typecast(d: &mut Duel, c: CardId) -> i64 {
    // return (c:GetType()&TYPE_FUSION+TYPE_RITUAL+TYPE_SYNCHRO+TYPE_XYZ)
    (card::get_type(d, c) & (TYPE_FUSION | TYPE_RITUAL | TYPE_SYNCHRO | TYPE_XYZ)) as i64
}
fn filter1(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsFaceup() and c:IsMonster() and c:IsAbleToHand() and c:IsCanBeEffectTarget(e)
    card::is_faceup(d, c) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None) && card::is_can_be_effect_target(d, c, Some(e))
}
fn filter2(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsTrap() and c:IsAbleToHand() and c:IsCanBeEffectTarget(e)
    card::is_trap(d, c) && card::is_able_to_hand(d, c, None) && card::is_can_be_effect_target(d, c, Some(e))
}
fn filter3(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsSpell() and c:GetCode()~=id and c:IsAbleToHand() and c:IsCanBeEffectTarget(e)
    card::is_spell(d, c) && card::get_code(d, c)[0] != ID && card::is_able_to_hand(d, c, None) && card::is_can_be_effect_target(d, c, Some(e))
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    let tp = a.tp;
    let e = a.e;
    // local g=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&cfilter), tp, LOCATION_MZONE, 0, NO_EX);
    // local ct=g:GetClassCount(s.typecast)
    let ct = group::get_class_count(d, &g, &|d, c| typecast(d, c)) as i64;
    // local g1=Duel.GetMatchingGroup(s.filter1,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,nil,e)
    let mut g1 = duel::get_matching_group(d, Some(&|d, c| filter1(d, c, e)), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, NO_EX);
    // local g2=Duel.GetMatchingGroup(s.filter2,tp,LOCATION_GRAVE,0,nil,e)
    let mut g2 = duel::get_matching_group(d, Some(&|d, c| filter2(d, c, e)), tp, LOCATION_GRAVE, 0, NO_EX);
    // local g3=Duel.GetMatchingGroup(s.filter3,tp,LOCATION_GRAVE,0,nil,e)
    let mut g3 = duel::get_matching_group(d, Some(&|d, c| filter3(d, c, e)), tp, LOCATION_GRAVE, 0, NO_EX);
    // if chk==0 then return (ct>1 and #g1>0) or (ct>2 and #g2>0) or (ct>3 and #g3>0) end
    if a.chk == Some(0) {
        return (ct > 1 && !g1.is_empty()) || (ct > 2 && !g2.is_empty()) || (ct > 3 && !g3.is_empty());
    }
    // local tg=Group.CreateGroup()
    let mut tg = CardSet::new();
    // local off=0
    // repeat ... until off<3 or not Duel.SelectYesNo(tp,aux.Stringid(id,3))
    loop {
        // local ops={}
        // local opval={}
        let mut ops: Vec<u64> = Vec::new();
        let mut opval: Vec<i64> = Vec::new();
        // off=1
        let mut off: usize = 1;
        // if ct>1 and #g1>0 then ops[off]=aux.Stringid(id,0) opval[off-1]=1 off=off+1 end
        if ct > 1 && !g1.is_empty() {
            ops.push(aux::stringid(ID, 0));
            opval.push(1);
            off += 1;
        }
        // if ct>2 and #g2>0 then ... opval 2
        if ct > 2 && !g2.is_empty() {
            ops.push(aux::stringid(ID, 1));
            opval.push(2);
            off += 1;
        }
        // if ct>3 and #g3>0 then ... opval 3
        if ct > 3 && !g3.is_empty() {
            ops.push(aux::stringid(ID, 2));
            opval.push(3);
            off += 1;
        }
        // local op=Duel.SelectOption(tp,table.unpack(ops))
        let op = duel::select_option(d, tp, true, &ops);
        // if opval[op]==1 then ...
        let v = opval[op as usize];
        if v == 1 {
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
            let sg = group::select(d, &g1, tp, 1, 1, false, NO_EX).nil();
            group::add_card(&mut tg, &sg);
            g1 = CardSet::new();
        } else if v == 2 {
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
            let sg = group::select(d, &g2, tp, 1, 1, false, NO_EX).nil();
            group::add_card(&mut tg, &sg);
            g2 = CardSet::new();
        } else {
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
            let sg = group::select(d, &g3, tp, 1, 1, false, NO_EX).nil();
            group::add_card(&mut tg, &sg);
            g3 = CardSet::new();
        }
        if off < 3 {
            break;
        }
        if !duel::select_yes_no(d, tp, aux::stringid(ID, 3)) {
            break;
        }
    }
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, &tg);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,tg,#tg,0,0)
    let n = tg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(tg), n, 0, 0);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
    // local sg=g:Filter(Card.IsRelateToEffect,nil,e)
    let sg = group::filter(d, &g, &|d, c| card::is_relate_to_effect(d, c, a.e), NO_EX);
    // if #sg>0 then
    if !sg.is_empty() {
        // Duel.SendtoHand(sg,nil,REASON_EFFECT)
        duel::send_to_hand(d, &sg, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,sg)
        duel::confirm_cards(d, 1 - a.tp, &sg);
    }
}
