//! Magicalibra
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 43857222;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum properties
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // Change levels
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetTarget(s.lvtg)
    effect::set_target(d, e1, Some(cost_target(lvtg)));
    // e1:SetOperation(s.lvop)
    effect::set_operation(d, e1, Some(operation(lvop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.lvfilter(c,e)`
fn lvfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsFaceup() and c:HasLevel() and c:IsCanBeEffectTarget(e)
    card::is_faceup(d, c) && card::has_level(d, c) && card::is_can_be_effect_target(d, c, Some(e))
}
/// The rescon of `aux.SelectUnselectGroup(g,e,tp,2,2,function(sg) return sg:IsExists(Card.IsLevelAbove,1,nil,lv+1) end,...)`
fn lv_rescon(d: &mut Duel, sg: &CardSet, lv: i64) -> (bool, bool) {
    (group::is_exists(d, sg, &|d, x| card::is_level_above(d, x, (lv + 1) as u32), 1, NO_EX), false)
}
/// `s.lvtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn lvtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local g=Duel.GetMatchingGroup(s.lvfilter,tp,LOCATION_MZONE,0,nil,e)
    let g = duel::get_matching_group(d, Some(&|d, c| lvfilter(d, c, a.e)), tp, LOCATION_MZONE, 0, NO_EX);
    // local _,hlv=g:GetMaxGroup(Card.GetLevel)
    let hlv = group::get_max_group(d, &g, &|d, c| card::get_level(d, c) as i64).map(|(_, v)| v);
    // if chk==0 then return #g>1 and hlv>1 end
    if a.chk == Some(0) {
        return g.len() > 1 && hlv.nil() > 1;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_LVRANK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_LVRANK as u64);
    // local lv=Duel.AnnounceLevel(tp,1,math.min(hlv-1,6))
    let lv = duel::announce_number_range(d, tp, Some(1), Some((hlv.nil() - 1).min(6) as u32), &[]).nil().0 as i64;
    // local tg=aux.SelectUnselectGroup(g,e,tp,2,2,function(sg)return sg:IsExists(Card.IsLevelAbove,1,nil,lv+1)end,1,tp,HINTMSG_FACEUP)
    let tg = aux::select_unselect_group(
        d,
        &g,
        Some(a.e),
        tp,
        Some(2),
        Some(2),
        Some(&|d, sg, _e, _tp, _mg, _c| lv_rescon(d, sg, lv)),
        1,
        tp,
        Some(HINTMSG_FACEUP as u64),
        None,
        None,
        false,
    )
    .unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg);
    // Duel.SetTargetParam(lv)
    duel::set_target_param(d, lv as u32);
    true
}
/// `s.lvop(e,tp,eg,ep,ev,re,r,rp)`
fn lvop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tg=Duel.GetTargetCards(e):Filter(Card.IsFaceup,nil)
    let targets = duel::get_target_cards(d, a.e).nil();
    let tg = group::filter(d, &targets, &|d, x| card::is_faceup(d, x), NO_EX);
    // if not c:IsRelateToEffect(e) or #tg<1 then return end
    if !card::is_relate_to_effect(d, c, a.e) || tg.is_empty() {
        return;
    }
    // local lv=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let lv = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int();
    // local reset=RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_END
    let reset = RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_END;
    if tg.len() == 1 {
        // local tc=tg:GetFirst()
        let tc = group::get_first(d, &tg).nil();
        // if tc:GetLevel()<=lv then return end
        if (card::get_level(d, tc) as i64) <= lv {
            return;
        }
        // tc:UpdateLevel(-lv,reset,c)
        card::update_level(d, tc, -lv, Some(reset), Some(c));
    } else if tg.len() == 2 {
        // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,0))
        duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 0));
        // local rd=tg:FilterSelect(tp,Card.IsLevelAbove,1,1,nil,lv+1):GetFirst()
        let rd_group = group::filter_select(d, &tg, tp, &|d, x| card::is_level_above(d, x, (lv + 1) as u32), 1, 1, false, NO_EX);
        let rd = group::get_first(d, &rd_group.nil());
        // if rd and rd:UpdateLevel(-lv,reset,c)~=0 then
        if let Some(rd) = rd {
            if card::update_level(d, rd, -lv, Some(reset), Some(c)) != 0 {
                // (tg-rd):GetFirst():UpdateLevel(lv,reset,c)
                let mut rest = tg.clone();
                group::remove_card(&mut rest, &CardSet::from_card(rd));
                let other = group::get_first(d, &rest).nil();
                card::update_level(d, other, lv, Some(reset), Some(c));
            }
        }
    }
}
